<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const name = ref("");
const password = ref("");

const go = async () => {
    const id = await sha256(name.value + password.value);
    console.log('sending request');
    await fetch(`${import.meta.env.VITE_API_URL}/api/auth`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            id,
            name: name.value,
            description: 'What are you trying to accomplish?',
            password: password.value
        })
    }).then(res => {
        console.log('got res');
        if (!res.ok) {
            console.log(res);
            throw new Error(res.statusText);
        }
        return res.json();
    }).then(data => {
        console.log('parsed');
        sessionStorage.setItem('token', `Bearer ${data.token}`);
        router.push({
            path: '/' + data.data.id,
        })
    })
    .catch(error => {
        console.error('Error:', error);
    });
}

async function sha256(message: string) {
    const msgBuffer = new TextEncoder().encode(message);

    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);

    const hashArray = Array.from(new Uint8Array(hashBuffer));

    const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    return hashHex;
}

const title = ref("Keep yourself on track to your goals. Everyday.");  
const subtitle = ref("Track your progress easily, without the hassle of a complicated app. Simply create a board, and start tracking your progress.");

</script>
<template>
    <div class="w-screen h-screen flex flex-col items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900">
        <div class="flex w-full py-12 md:py-24 lg:py-32 xl:py-48 items-center justify-center">
            <div class="w-full container px-4 md:px-6">
                <div class="flex flex-col items-center space-y-4 text-center">
                    <div class="space-y-2">
                        <h1
                            class="text-4xl font-bold tracking-tighter sm:text-5xl md:text-6xl lg:text-7xl text-gray-900 dark:text-gray-100">
                            {{ title }}
                        </h1>
                        <p class="mx-auto max-w-[700px] text-lg text-gray-500 md:text-xl dark:text-gray-400">
                            {{ subtitle }}
                        </p>
                    </div>
                    <div class="w-full max-w-md">
                        <div class="flex flex-col space-y-4">
                            <label class="pt-4 dark:text-gray-300 text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 select-none">Create or Go to a board</label>
                            <input v-model="name" class="w-full flex h-9 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium dark:text-white placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50" placeholder="Board Name" type="text" />
                            <input v-model="password" class="w-full flex h-9 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium dark:text-white placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50" placeholder="Board Password" type="password" />
                            <button @click="go" class="w-full py-2 bg-gray-900 text-white dark:bg-gray-100 dark:text-gray-900 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50">
                                Get Started
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
