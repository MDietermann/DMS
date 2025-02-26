<script setup lang="ts">
import { useRouter } from 'vue-router';
import { ref } from 'vue';

const data = ref({
    host: "",
    port: 3306,
    user: "",
    password: ""
})

const router = useRouter();

const abort = () => {
    router.push("/");
}
</script>

<template>
    <div class="flex flex-col">
        <div class="flex flex-row gap-4 p-4">
            <div class="w-1/2 flex flex-col gap-2">
                <label>
                    Host-Adresse:
                    <input v-model="data.host" type="text" name="host" id="host-input" class="form-control" placeholder="192.168.0.1">
                </label>
                <label>
                    Port:
                    <input v-model="data.port" type="number" name="port" id="port-input" class="form-control" placeholder="3306">
                </label>
            </div>
            <div class="w-1/2 flex flex-col gap-2">    
                <label>
                    User:
                    <input v-model="data.user" type="text" name="user" id="user-input" class="form-control" placeholder="root">
                </label>
                <label>
                    Password:
                    <input v-model="data.password" type="password" name="password" id="password-input" class="form-control">
                </label>
            </div>
        </div>
        <div class="flex flex-row w-full justify-center gap-8">
            <button class="btn btn-success w-1/4">
                Verify Connection
            </button>
            <button class="btn btn-outline-warning w-1/4" @click="abort">
                Cancel
            </button>
        </div>
    </div>
</template>