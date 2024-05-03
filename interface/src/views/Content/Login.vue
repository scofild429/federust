	<template>
		<div class="container">
		<ValidateForm @form-submit="onFormSubmit">
			<div class=" boxxx">
			<ValidateEmail
				style="width: 550px"
				type="text"
				v-model="data.email"
				:rules="emailRules"
				placeholder="please input your email"
				ref="inputRef"
			></ValidateEmail>
			</div>
			<div class="mb-3 boxxx">
			<ValidatePass
				type="password"
				style="width: 550px"
				v-model="data.password"
				:rules="passwordRules"
				placeholder="please input your password"
				ref="passRef"
			></ValidatePass>
			</div>
			<template #submit >
			<span class="btn btn-warning boxxx">Submit</span>
			</template>
		</ValidateForm>
		</div>
	</template>
	<script lang="ts">
	import {defineComponent, reactive, ref} from "vue";
	import ValidateForm from "../../components/ValidForm/ValidateForm.vue";
	import ValidateEmail from "../../components/ValidForm/ValidateEmail.vue";
	import ValidatePass from "../../components/ValidForm/ValidatePass.vue";
	import {useStore} from "vuex";
	import { useRouter } from "vue-router";
	import {greet} from "liberal-federated-learning";
	// import { useCookie } from 'vue-cookie-next'
	
	export default defineComponent({
	name: "Login",
	components: {
		ValidateForm,
		ValidateEmail,
		ValidatePass,
	},
	setup() {
		const store = useStore();
		const router = useRouter();
		// const cookie = useCookie()
		const emailRules = store.state.Login.emailRules;
		const passwordRules = store.state.Login.passwordRules;
		const inputRef = ref<any>();
		const passRef = ref<any>();
		const data = reactive({
		email: "",
		password: "",
		});
		const userinfo = reactive({
		email: "",
		name: "",
		profileId:"",
		toemail: "",
		contactperson: "",
		fromemail: "",
		fromemailpassword:"",
		});
		const onFormSubmit = async () => {
			greet();
			console.log("submit boot");
		if (inputRef.value.validateInput() && passRef.value.validateInput()) {
		try {
		let dataResp = await fetch(import.meta.env.VITE_Domain+"/api/login", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify(data)
		});
		const json_data = await dataResp.json();
		const jwt = json_data["jwt"]
		//   cookie.setCookie("jwt", jwt);
		
		try {
			let dataResp = await fetch(import.meta.env.VITE_Domain+"/api/user", {
			method: "GET",
			credentials: 'include',
			});
			const json_data = await dataResp.json();
			userinfo.email = json_data["email"];
			userinfo.name = json_data["username"];
			userinfo.profileId = json_data["profileId"];
			userinfo.contactperson = json_data["contactperson"];
			userinfo.fromemail = json_data["fromemail"];
			userinfo.fromemailpassword = json_data["fromemailpassword"];
			userinfo.toemail = json_data["toemail"];
			store.dispatch("login",  userinfo);
			router.push("/");			 
		}catch(err) {
			console.log(err)
		}
		}catch(err) {
		console.log(err)
		}
		}
		};

		return {
		emailRules,
		passwordRules,
		onFormSubmit,
		inputRef,
		passRef,
		data,
		userinfo,
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
