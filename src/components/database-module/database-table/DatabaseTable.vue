
<script setup lang="ts">
const props = defineProps({
    databaseData: {
        type: Array,
        required: true
    }
})

console.log(props.databaseData);

</script>

<template>
    <table class="table">
        <thead>
            <tr>
                <th scope="col" v-for="(value, key) in databaseData[0]">
                    {{ key }}
                </th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="data in databaseData">
                <td v-for="value in data">
                    {{ value }}
                </td>
            </tr>
        </tbody>
    </table>
</template>

<style lang="css" scoped>
</style>
