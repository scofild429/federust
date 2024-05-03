import { currentUser, disabled } from './testData'

const state = () => ({
    users: currentUser,
    disable: disabled
})

const getter = {
}

const actions = {
}

const mutations = {
    login: (state: any) => {
	state.users.name = 'viking'
	state.users.isLogin = true
    }
}

export default {
    namespaced: true,
    state,
    getter,
    actions,
    mutations
}
