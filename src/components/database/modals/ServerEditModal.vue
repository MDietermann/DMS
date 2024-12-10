<script setup lang="ts">
import { ref } from 'vue';
import type { DatabaseServer } from '../../../types/database';
import { useDatabaseStore } from '../../../stores/database';

const props = defineProps<{
    server: DatabaseServer;
    databaseType: string;
}>();

const emit = defineEmits<{
    close: [];
}>();

const store = useDatabaseStore();
const editedServer = ref<DatabaseServer>({
    ip: props.server.ip,
    users: { ...props.server.users }
});

function handleSave() {
    store.updateServer(props.databaseType as any, props.server.ip, editedServer.value);
    emit('close');
}
</script>

<template>
    <div class="modal fade show d-block" tabindex="-1">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title">Edit Server</h5>
                    <button type="button" class="btn-close" @click="emit('close')" />
                </div>
                <div class="modal-body">
                    <div class="mb-3">
                        <label class="form-label">IP Address</label>
                        <input v-model="editedServer.ip" type="text" class="form-control" placeholder="192.168.0.1" />
                    </div>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" @click="emit('close')">
                        Cancel
                    </button>
                    <button type="button" class="btn btn-primary" @click="handleSave">
                        Save Changes
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
