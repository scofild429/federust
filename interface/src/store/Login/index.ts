import { emailReg, emailRules, passwordRules, GetPass } from "./testData";

const state = () => ({
  emailReg,
  emailRules,
  passwordRules,
  GetPass,
});

const getter = {};

const actions = {};

const mutations = {
  ifpass: (state: any, pass: boolean) => {
    state.GetPass.push(pass);
  },
};

export default {
  namespaced: true,
  state,
  getter,
  actions,
  mutations,
};
