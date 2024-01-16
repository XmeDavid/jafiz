CREATE TABLE jafiz
(
    id          VARCHAR(255)                NOT NULL,
    name        VARCHAR(255)                NOT NULL,
    password    VARCHAR(255)                NOT NULL,
    description VARCHAR(255)    DEFAULT ''  NOT NULL,
    january     INT             DEFAULT 0   NOT NULL,
    february    INT             DEFAULT 0   NOT NULL,
    march       INT             DEFAULT 0   NOT NULL,
    april       INT             DEFAULT 0   NOT NULL,
    may         INT             DEFAULT 0   NOT NULL,
    june        INT             DEFAULT 0   NOT NULL,
    july        INT             DEFAULT 0   NOT NULL,
    august      INT             DEFAULT 0   NOT NULL,
    september   INT             DEFAULT 0   NOT NULL,
    october     INT             DEFAULT 0   NOT NULL,
    november    INT             DEFAULT 0   NOT NULL,
    december    INT             DEFAULT 0   NOT NULL,
    PRIMARY KEY (id)
);