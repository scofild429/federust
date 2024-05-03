<template>
    <form @submit.prevent="submit" >
	<input
	    v-model="data.username"
	    style="width: 50%"
	    type="text"
	    class="form-control boxxx"
	    placeholder="Username, required"
	    required
	/>
	<br />
	<input
	    v-model="data.email"
	    style="width: 50%"
	    type="email"
	    class="form-control boxxx"
	    placeholder="Email, required, unique"
	    required
	/>
	<br />
	<input
	    v-model="data.password"
	    style="width: 50%"
	    type="password"
	    class="form-control boxxx"
	    placeholder="Password, required"
	    required
	/>
	<br />
  	<button class="btn boxxx btn-primary" type="submit">Submit</button>
    </form>
</template>

<script lang="ts">
 import {defineComponent, reactive} from "vue";
 import {useRouter} from "vue-router";
 export default defineComponent({
     name: "Register",
     setup() {
	 const router = useRouter();
	 const data = reactive({
	     email: "",
	     username: "",
	     password: "",
	 });	 
	 const submit = async() => {
	     try {
	       let dataResp = await fetch(import.meta.env.VITE_Domain+"/api/register", {
		 method: "POST",
		 headers: { "Content-Type": "application/json" },
		 body: JSON.stringify(data)
		 });		 
		 const json_data = await dataResp.json();
		 await router.push("/login");
	     }catch(err) {
		 console.log(err)
	     }
	 };
	 return {
		data,
	     submit,
	 };
     },
 });
</script>

<style scoped>
 .boxxx {
     margin: 0 auto;
     display: table;
 }
</style>

