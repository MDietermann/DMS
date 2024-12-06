<script setup lang="ts">
import DatabaseModule from "../components/database-module/DatabaseModule.vue";
import { acceptedFileTypesExport, acceptedFileTypesImport } from "../mock/filetypes";
import { resolveUserDataNames } from "../scripts/dbResolver";
import Section from "../components/section/Section.vue";
import { useEmployeeStore } from "../stores/employee";
import Login from "../components/login/Login.vue";
import { dbMock } from "../mock/db-mock";
import { ref, watch } from "vue";

const employeeStore = useEmployeeStore();
const isLoggedIn = ref(employeeStore.isLoggedIn);

watch(
    () => employeeStore.isLoggedIn,
    () => {
        isLoggedIn.value = employeeStore.isLoggedIn;
    }
);

const validDataTypes = (typesArray: string[]) => {
    return typesArray.join(", ");
};
</script>

<template>
    <template v-if="!isLoggedIn">
        <Login />
    </template>
    <template v-else>
        <div id="welcome" class="home-body">
            <p class="display-6">Welcome to the DMS!</p>
            <span class="lead">
                <strong>D</strong>atabase <strong>M</strong>anagement <strong>S</strong>ystem
            </span>
            <br />
            <p>
                Your tool for converting database tables to
                <strong>{{ validDataTypes(acceptedFileTypesExport) }}</strong> and importing
                <strong>{{ validDataTypes(acceptedFileTypesImport) }}</strong> into database
                tables!
            </p>
        </div>
        <hr />
        <Section class="row" title="Databases">
            <DatabaseModule :databases="dbMock" />
        </Section>
        <Section class="row" title="User Data">
            <div class="row">
                <table class="table">
                    <tbody>
                        <tr v-for="(val, key) in employeeStore.employee">
                            <th scope="row">
                                {{ resolveUserDataNames(key) }}
                            </th>
                            <td>
                                {{ val }}
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="row">
                <p>Change user data?</p>
                <button type="button" class="btn btn-outline-primary">Change</button>
            </div>
        </Section>
    </template>
</template>

<style lang="css" scoped>
.home-body {
    margin: 32px 0;
}
</style>
