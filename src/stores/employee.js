import { defineStore } from "pinia";

export const useEmployeeStore = defineStore("employee", {
    state: () => ({
        loggedIn: false,
        employee: {
            id: -1,
            first_name: "",
            last_name: "",
            email: "",
            position: ""
        }
    }),
    actions: {
        login(employee) {
            this.loggedIn = true
            this.employee = {
                id: employee.id,
                first_name: employee['first_name'],
                last_name: employee['last_name'],
                email: employee['email'],
                position: employee['position']
            }
        },
        logout() {
            this.employee = null
            this.loggedIn = false
        }
    },
    getters: {
        isLoggedIn: (state) => state.loggedIn,
        getEmployee: (state) => state.employee
    }
})
