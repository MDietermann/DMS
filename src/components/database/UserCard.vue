<script setup lang="ts">
import { ref } from 'vue';
import type { TableUser } from '../../types/database';
import TableList from './TableList.vue';
import UserEditModal from './modals/UserEditModal.vue';
import PasswordPrompt from './modals/PasswordPrompt.vue';
import { useDatabaseStore } from '../../stores/database';

const props = defineProps<{
    username: string;
    userData: TableUser;
    databaseType: string;
    serverIp: string;
    isAdmin?: boolean;
}>();

const store = useDatabaseStore();
const showEditModal = ref(false);
const showPasswordPrompt = ref(false);
const showPassword = ref(false);
const passwordVerified = ref(false);

function handleDelete() {
    if (confirm('Are you sure you want to delete this user?')) {
        store.deleteUser(props.databaseType as any, props.serverIp, props.username);
    }
}

function handlePasswordVerified() {
    passwordVerified.value = true
}

function togglePasswordVisibility() {
    if (!showPassword.value) {
        showPasswordPrompt.value = true;
    } else {

        showPassword.value = false;
    }
}

function handleShowEditModal() {
    console.log(passwordVerified.value);
    if (passwordVerified.value) {
        showEditModal.value = true;
    } else {
        showPasswordPrompt.value = true;
    }
}

function handleCloseEditModal() {
    showEditModal.value = false;
    passwordVerified.value = false;
}
</script>

<template>
    <div class="card">
        <div class="card-body">
            <div class="flex justify-between items-center mb-3">
                <h3 class="card-title m-0">{{ username }}</h3>
                <div class="flex gap-2">
                    <button class="btn btn-outline-primary btn-sm" @click="handleShowEditModal">
                        <i class="bi bi-pencil"></i>
                    </button>
                    <button v-if="isAdmin" class="btn btn-outline-danger btn-sm" @click="handleDelete">
                        <i class="bi bi-trash"></i>
                    </button>
                </div>
            </div>
            <div class="text-sm text-muted mb-2">
                <button class="btn btn-outline-secondary btn-sm" @click="togglePasswordVisibility">
                    <i class="bi" :class="showPassword ? 'bi-eye-slash' : 'bi-eye'"></i>
                    {{ showPassword ? 'Hide Password' : 'Show Password' }}
                </button>
                <span v-if="showPassword" class="ms-2">
                    <i class="bi bi-key me-1"></i>{{ userData.password }}
                </span>
            </div>
            <TableList :tables="userData.tables" />
        </div>

        <UserEditModal v-if="showEditModal" :username="username" :user-data="userData" :database-type="databaseType"
            :server-ip="serverIp" @close="handleCloseEditModal" />

        <PasswordPrompt v-if="showPasswordPrompt" :username="username" :correct-password="userData.password"
            :server-ip="serverIp" :database-type="databaseType" @verified="handlePasswordVerified"
            @show-password="showPassword = true" @close="showPasswordPrompt = false" />
    </div>
</template>
