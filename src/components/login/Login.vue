<script setup lang="ts">
import { defineEmits, InputHTMLAttributes } from 'vue';
const emit = defineEmits(['login'])

const validate = () => {
    console.log("validating...");
    const username: HTMLInputElement | null = document.getElementById('username_login') as HTMLInputElement
    const password: HTMLInputElement | null = document.getElementById('password_login') as HTMLInputElement

    console.log(`Vals: ${username?.value}, ${password?.value}`);


    if (username?.value && password?.value) {
        emit('login')
    }
}

</script>

<template>
    <div class="body">
        <div class="backdrop">
            <div class="header">
                <div>DMS<span>-Manager</span></div>
            </div>
            <br>
            <div class="login">
                <input id="username_login" type="text" placeholder="username" name="user" value=""><br>
                <input id="password_login" type="password" placeholder="password" name="password" value=""><br>
                <input type="button" value="Login" @click="validate()">
            </div>
        </div>
    </div>
</template>

<style scoped>
@import url(https://fonts.googleapis.com/css?family=Exo:100,200,400);
@import url(https://fonts.googleapis.com/css?family=Source+Sans+Pro:700,400,300);

.body {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-image: url(../../assets/Banniere-blog-yggdrasil.png);
    background-position: 50%;
    background-size: cover;
    z-index: 0;
}

.backdrop {
    position: absolute;
    height: 100%;
    width: 100%;
    backdrop-filter: blur(12px);
}

.header {
    position: absolute;
    top: calc(50% - 35px);
    left: calc(50% - 255px);
    z-index: 2;
}

.header div {
    float: left;
    color: #fff;
    font-family: 'Exo', sans-serif;
    font-size: 35px;
    font-weight: 200;
}

.header div span {
    color: #5379fa !important;
}

.login {
    position: absolute;
    top: calc(50% - 75px);
    left: calc(50% - 50px);
    height: 150px;
    width: 350px;
    padding: 10px;
    z-index: 2;
}

.login input[type=text] {
    width: 250px;
    height: 30px;
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.6);
    border-radius: 2px;
    color: #fff;
    font-family: 'Exo', sans-serif;
    font-size: 16px;
    font-weight: 400;
    padding: 4px;
}

.login input[type=password] {
    width: 250px;
    height: 30px;
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.6);
    border-radius: 2px;
    color: #fff;
    font-family: 'Exo', sans-serif;
    font-size: 16px;
    font-weight: 400;
    padding: 4px;
    margin-top: 10px;
}

.login input[type=button] {
    width: 260px;
    height: 35px;
    background: #fff;
    border: 1px solid #fff;
    cursor: pointer;
    border-radius: 2px;
    color: #a18d6c;
    font-family: 'Exo', sans-serif;
    font-size: 16px;
    font-weight: 400;
    padding: 6px;
    margin-top: 10px;
}

.login input[type=button]:hover {
    opacity: 0.8;
}

.login input[type=button]:active {
    opacity: 0.6;
}

.login input[type=text]:focus {
    outline: none;
    border: 1px solid rgba(255, 255, 255, 0.9);
}

.login input[type=password]:focus {
    outline: none;
    border: 1px solid rgba(255, 255, 255, 0.9);
}

.login input[type=button]:focus {
    outline: none;
}

::-webkit-input-placeholder {
    color: rgba(255, 255, 255, 0.6);
}

::-moz-input-placeholder {
    color: rgba(255, 255, 255, 0.6);
}
</style>
