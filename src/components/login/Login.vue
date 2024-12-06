<script setup lang="ts">
import { defineEmits, InputHTMLAttributes, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core'
import { useEmployeeStore } from '../../stores/employee'

const employeeStore = useEmployeeStore()
const emit = defineEmits(['login']);
const error = ref("")

/**
 * Handles the login process.
 * Fetches the current values from the input fields, makes a call to the tauri login function
 * with the given employee id, and then calls the login function from the employee store.
 * The result of this call is then evaluated and if it's a successful login, the login event is emitted.
 * Otherwise, an error message is set.
 */
const validateLogin = () => {
    // Get input elements for employee ID and password
    const employeeIdInput = document.getElementById('employee_id_login') as HTMLInputElement;
    const passwordInput = document.getElementById('password_login') as HTMLInputElement;
    // Check if both input fields have values
    if (employeeIdInput.value && passwordInput.value) {
        // Parse the employee ID into a number
        const numericId = parseInt(employeeIdInput.value);
        // Call the tauri login function with the employee ID
        invoke<number>('login', { employeeId: numericId })
            .then((responseCode) => {
                // Call the login function from the employee store with the response code
                const result = employeeStore.login(responseCode);
                // Check if the login was successful
                if (result.Code === 200) {
                    // Emit the login event
                    emit('login');
                } else {
                    // Set the error message if login was unsuccessful
                    error.value = result.Message || 'Unknown error occurred during login.';
                }
            })
            .catch((err) => {
                // Log any errors that occur during the process
                console.error(err);
            });
    }
};

</script>

<template>
    <div class="body">
        <div class="backdrop">
            <div class="header">
                <div>DMS<span>-Manager</span></div>
            </div>
            <br>
            <div class="login">
                <input id="employee_id_login" type="text" placeholder="Mitarbeiter-ID" name="employee_id" value=""><br>
                <input id="password_login" type="password" placeholder="Passwort" name="password" value=""><br>
                <input type="button" value="Login" @click="validate()">
                <p class="text-warning" id="error">
                    {{ error }}
                </p>
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
