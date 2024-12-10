<script setup lang="ts">
import { ref } from 'vue';
import type { TableUser } from '../../../types/database';
import { useDatabaseStore } from '../../../stores/database';

const props = defineProps<{
    username: string;
    userData: TableUser;
    databaseType: string;
    serverIp: string;
}>();

const emit = defineEmits<{
    close: [];
}>();

const store = useDatabaseStore();
const editedUser = ref<TableUser>({
    tables: [...props.userData.tables],
    password: props.userData.password
});

const newTable = ref('');

function addTable() {
    if (newTable.value && !editedUser.value.tables.includes(newTable.value)) {
        editedUser.value.tables.push(newTable.value);
        newTable.value = '';
    }
}

function removeTable(table: string) {
    editedUser.value.tables = editedUser.value.tables.filter(t => t !== table);
}

function handleSave() {
    store.updateUser(
        props.databaseType as any,
        props.serverIp,
        props.username,
        editedUser.value
    );
    emit('close');
}
</script>

<template>
    <div class="modal fade show d-block" tabindex="-1">
        <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title">Edit User: {{ username }}</h5>
                    <button type="button" class="btn-close" @click="emit('close')" />
                </div>
                <div class="modal-body">
                    <div class="mb-3">
                        <label class="form-label">Password</label>
                        <input v-model="editedUser.password" type="text" class="form-control"
                            placeholder="Enter password" />
                    </div>
                    <div class="mb-3">
                        <label class="form-label">Tables</label>
                        <div class="input-group mb-2">
                            <input v-model="newTable" type="text" class="form-control" placeholder="Add new table"
                                @keyup.enter="addTable" />
                            <button class="btn btn-outline-secondary" type="button" @click="addTable">
                                Add
                            </button>
                        </div>
                        <div class="d-flex flex-wrap gap-2">
                            <span v-for="table in editedUser.tables" :key="table"
                                class="badge bg-secondary d-flex align-items-center">
                                {{ table }}
                                <button class="btn-close btn-close-white ms-2" style="font-size: 0.5em"
                                    @click="removeTable(table)" />
                            </span>
                        </div>
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
