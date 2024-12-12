<script setup lang="ts">
import { ref } from 'vue';
import { useDatabaseStore } from '../../../stores/database';

const props = defineProps<{
    username: string;
    correctPassword: string;
    serverIp: string;
    databaseType: string;
}>();

const emit = defineEmits<{
    verified: [];
    showPassword: [];
    close: [];
}>();

const store = useDatabaseStore();
const password = ref('');
const error = ref('');

function verifyPassword() {
    const server = store.getServer(props.databaseType, props.serverIp);

    if (
        password.value === props.correctPassword ||
        (server?.adminPassword === password.value && server?.adminUser)
    ) {
        error.value = '';
        if (password.value === props.correctPassword) {
            emit('showPassword');
            emit('verified');
        }
        emit('close');
    } else {
        error.value = 'Invalid password';
    }
}
</script>

<template>
    <div class="modal fade show d-block" tabindex="-1">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title">Enter Password</h5>
                    <button type="button" class="btn-close" @click="emit('close')" />
                </div>
                <div class="modal-body">
                    <div class="mb-3">
                        <label class="form-label">Password for {{ username }}</label>
                        <input v-model="password" type="password" class="form-control" placeholder="Enter password"
                            @keyup.enter="verifyPassword" />
                        <div v-if="error" class="text-danger mt-2">
                            {{ error }}
                        </div>
                    </div>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" @click="emit('close')">
                        Cancel
                    </button>
                    <button type="button" class="btn btn-primary" @click="verifyPassword">
                        Verify
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
