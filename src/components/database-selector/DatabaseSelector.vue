<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps({
    headline: {
        type: String,
    },
    tableContent: {
        type: Object,
        reqired: true
    }
})
const collapseName = (suffix: string) => {
    return `accordion-collapse-${suffix}`
}

function check(id:string) {
    const checkedItem: HTMLInputElement = document.getElementById(id) as HTMLInputElement
    const val = checkedItem.checked
    console.log(id, val);
}
</script>

<template>
    <h2>{{ headline }}</h2>
    <div class="accordion accordion-flush container-sm" id="accordionFlush">
        <div class="accordion-item" v-for="(data, index) in tableContent">
            <h2 class="accordion-header">
                <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse"
                    :data-bs-target="'#' + collapseName(index)" aria-expanded="false"
                    :aria-controls="collapseName(index)">
                    {{ index }}
                </button>
            </h2>
            <div :id="collapseName(index)" class="accordion-collapse collapse" data-bs-parent="#accordionFlush">
                <div class="accordion-body">
                    <table class="table">
                        <thead>
                            <tr>
                                <td scope="col">#</td>
                                <td scope="col">User</td>
                                <td scope="col">Tables</td>
                                <td scope="col">Select</td>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="(db) in data">
                                <th scope="row">{{ db['ip'] }}</th>
                                <td>{{ db['user'] }}</td>
                                <td>
                                    <table class="table">
                                        <tr v-for="(table, index) in db['tables']">
                                            <td>{{ table }}</td>
                                        </tr>
                                    </table>
                                </td>
                                <td>
                                    <div class="form-check">
                                        <input class="form-check-input" type="checkbox" value="" :id="`check_${index}_${db['user']}`" @click="check(`check_${index}_${db['user']}`)">
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped></style>
