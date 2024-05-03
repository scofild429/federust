<template>
    <div class="about">
	<br/>
	<div class="container">
	    <div class="row justify-content-evenly">
		<div class="col-4">
		    <button type="button" @click.prevent="submitLogout" class="btn btn-warning">
			yes, and go to login
		    </button>
		</div>
		<div class="col-4">
		    <button type="button" @click.prevent="submitHome" class="btn btn-success">
			Or to the home site and stay for a while!
		    </button>
		</div>
	    </div>
	</div>
    </div>
</template>

<script lang="ts">
 import {defineComponent} from "vue";
 import {useRouter} from "vue-router";
 import {useStore} from "vuex";
 import { useCookie } from 'vue-cookie-next'

 export default defineComponent({
     name: "Logout",
     setup() {
	 const router = useRouter();
	 const store = useStore();
	 const cookie = useCookie()
	 const submitLogout = async () => {
	     try {
	       let dataResp = await fetch(import.meta.env.VITE_Domain+"/api/logout", {
		 method: "POST",
		 credentials: 'include',
	       });
		 cookie.removeCookie("jwt")
	     }catch(err) {
		 console.log(err)
	     }
	     await store.dispatch("logout");
	     await router.push("/login");
	 };

	 const submitHome = () => {
	     router.push("/");
	 };
	 return {
	     submitLogout,
	     submitHome,
	 };
     },
 });
</script>
