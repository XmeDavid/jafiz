<script setup lang="ts">
import { ref, Ref } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const id = route.params.id as string;

type Data = {
    id: string
    name: string
    description: string
    months: boolean[][]
    monthNames: string[]
} | null;

const data: Ref<Data> = ref(null);
const loading = ref(true);
const gradientButton: Ref<HTMLElement | null> = ref(null);
const authenticated: Ref<boolean> = ref(false);


const validateToken = async () => {
    return await fetch(`/api/auth`, {
        headers: {
            'Authorization': sessionStorage.getItem('token') || ''
        }
    }).then(res => {
        if(res.status === 200) {
            authenticated.value = true
        } else {
            authenticated.value = false
        }
    }).catch(_ => {
        authenticated.value = false
    })
}
validateToken();

const fetchBoard = async (id: string) => {
    loading.value = true
    return await fetch(`/api/${id}`, {
        headers: {
            'Authorization': sessionStorage.getItem('token') || ''
        }
    }).then(res => res.json()).then(data => {
        updateData(data)
    }).catch(err => {
        console.log(err);
        return null;
    })
}
fetchBoard(id)

const updateData = (res: any) => {
    data.value = {
        ...res,
        months: [res.january, res.february, res.march, res.april, res.may, res.june, res.july, res.august, res.september, res.october, res.november, res.december],
        monthNames: ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December']
    }
    loading.value = false
}

const checkIn = async () => {
    loading.value = true
    return await fetch(`/api/check_in`, {
        method: 'POST',
        headers: {
            'Authorization': sessionStorage.getItem('token') || ''
        }
    }).then(res => res.json()).then(data => {
        updateData(data)
    }).catch(err => {
        console.log(err);
        return null;
    })
}

const getEmptyDays = (monthIndex: number) => {
    const year = new Date().getFullYear();
    const dayIndex = new Date(`${monthIndex + 1} 01, ${year}`).getDay();
    return Array.from({ length: dayIndex }, (_, i) => i);
}

const updateGradientPosition = (e: MouseEvent) => {
    if (!gradientButton.value) return;
    const rect = gradientButton.value.getBoundingClientRect();
    const x = (e.clientX - rect.left) / gradientButton.value.offsetWidth * 100;
    const y = (e.clientY - rect.top) / gradientButton.value.offsetHeight * 100;
    // @ts-ignore
    gradientButton.value.children[0].style.backgroundPosition = `${x}% ${y}%`;
};

const editingMode = ref(false);

const updateDescription = async () => {
    loading.value = true
    editingMode.value = false
    return await fetch(`/api/description`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': sessionStorage.getItem('token') || ''
        },
        body: JSON.stringify({
            description: data.value ? data.value.description : ''
        })
    }).catch(err => {
        console.log(err);
        return null;
    }).finally(() => {
        loading.value = false
    })
}

</script>
<template>
    <div v-if="!loading && data" class="flex flex-col items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900">
        <div class="fixed top-0 left-0 w-full px-10 py-2 bg-white dark:bg-gray-800 shadow-md">
            <router-link to="/" class="text-2xl font-bold text-gray-900 dark:text-gray-100 hover:text-green-500">
                Home
            </router-link>
        </div>
        <div class="flex justify-center items-center w-full px-10 py-10 mt-8">
            <div class="text-center">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">{{ data ? data.name : '' }}</h1>
                <div class="flex flex-row">
                    <h2
                        :class="[{'hidden': editingMode}]"
                        class="select-none text-2xl text-center text-gray-700 dark:text-gray-300 mr-2"
                    >
                        {{ data.description }}
                    </h2>
                    <input
                        v-model="data.description"
                        type="text"
                        :class="[{'hidden': !editingMode}]"
                        style="min-width: 0px;"
                        class="mr-2 mt-1 block w-fit text-2xl text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-green-500 focus:outline-none focus:ring-0 focus:border-green-600"
                    >
                    <svg v-if="!editingMode" @click="editingMode = !editingMode" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                        class="w-8 h-8 stroke-gray-400 dark:stroke-gray-600 hover:stroke-green-500 ">
                        <path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" />
                    </svg>
                    <svg v-if="editingMode" @click="updateDescription"  xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                        class="w-8 h-8 stroke-gray-400 dark:stroke-gray-600 hover:stroke-green-500 ">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 16.5V9.75m0 0 3 3m-3-3-3 3M6.75 19.5a4.5 4.5 0 0 1-1.41-8.775 5.25 5.25 0 0 1 10.233-2.33 3 3 0 0 1 3.758 3.848A3.752 3.752 0 0 1 18 19.5H6.75Z" />
                    </svg>

                </div>
                
            </div>
        </div>
        <div class="flex flex-col items-center justify-center w-full">

            <button v-if="authenticated" @click="checkIn" @mousemove="updateGradientPosition" ref="gradientButton"
                class="w-48 h-16 relative flex justify-center place-items-center items-center  mb-10 bg-green-400 rounded-lg group">
                <span
                    class="absolute inset-0 bg-gradient-to-r opacity-0 group-hover:opacity-100 transition-opacity  duration-300"
                    style="background-size: 200% 200%;"></span>

                <p class="z-50 text-xl font-bold text-white text-nowrap w-min h-min"> Check in</p>
            </button>

            <div class="flex flex-wrap gap-4 p-4 rounded-lg bg-white shadow-lg dark:bg-gray-800 max-w-screen-md mx-8 justify-center"> <!-- gap-4 for separation between months -->
                <div v-for="(month, monthIndex) in data.months" :key="monthIndex">
                    <span class="text-gray-600 dark:text-gray-400 p-1"> {{ data.monthNames[monthIndex] }}</span>
                    <div class="grid grid-rows-7 grid-flow-col  gap-1">
                        <div v-for="emptyDay in getEmptyDays(monthIndex)" :key="'empty-' + monthIndex + '-' + emptyDay"
                            class="w-4 h-4"></div>
                        <div v-for="(day, index) in month" :key="monthIndex + '-' + index" class="w-4 h-4 rounded-sm"
                            :class="{ 'bg-green-500': day, 'bg-gray-300 dark:bg-gray-600 shadow-inner': !day }">
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <div v-else class="flex items-center justify-center h-screen bg-gray-100 dark:bg-gray-900">
        <div class="animate-spin h-12 w-12 rounded-full border-t-2 border-gray-900 dark:border-gray-50" />
    </div>
</template>
<style scoped>
@keyframes rotateGradient {
    0% {
        background-position: 0% 50%;
    }

    50% {
        background-position: 100% 50%;
    }

    100% {
        background-position: 0% 50%;
    }
}

button {
    position: relative;
    overflow: hidden;
    /* Other styling */
}

button span {
    position: absolute;
    inset: 0;
    background: linear-gradient(to right, #6bff53, #0d6c08);
    background-size: 200% 200%;
    transition: background-position 0.5s, opacity 0.5s ease-in-out;
    opacity: 0;
}

button:hover span {
    opacity: 1;
    animation: rotateGradient 3s linear infinite;
}</style>
