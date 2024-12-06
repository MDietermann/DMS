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
            if (employee.id === -1) {
                return {
                    code: 401,
                    message: "Employee not found",
                };
            }
            this.loggedIn = true;
            this.employee = {
                id: employee.id,
                firstName: employee.firstName,
                lastName: employee.lastName,
                email: employee.email,
                position: employee.position,
            };
            return { code: 200 };
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
