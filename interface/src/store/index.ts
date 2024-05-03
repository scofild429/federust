import { createStore } from "vuex";
import Login from "./Login";
import Header from "./Header";



interface ProfileProps {
  toemail?: string | null;
  fromemail?: string | null;
  fromemailpassword?: string | null;
  contactperson?: string | null;
}

interface UserProps {
  isLogin?: boolean;
  name?: string | null;
  email?: string | null;
  profileId?: string | null;
}

const user: UserProps = {
  isLogin: JSON.parse(window.sessionStorage.getItem("userisLogin") || "false"),
  name: "DefaultName",
  email: "",
  profileId: "",
};

const profile: ProfileProps = {
  toemail: "",
  fromemail: "",
  contactperson: "",
  fromemailpassword: "",
};


export default createStore({
  state: {
    user: user,
    profile: profile,

  },
  mutations: {
    //  SET_AUTH: (state, userinfo: UserProps) => {
    //    state.user.isLogin = true;
    //    window.sessionStorage.setItem("userisLogin", "true");
    //    state.user.email = userinfo.email;
    //    state.user.name = userinfo.name;
    //    state.user.profileId = userinfo.profileId;
    //  },
  },
  actions: {
    //  login(content, userinfo: UserProps) {
    //    content.commit("SET_AUTH", userinfo);
    //  },

  },
  modules: {
    Login,
    Header,
  },
});
