<template>
  <nav class="navbar navbar-dark bg-primary justify-content-between mb-4">
    <router-link
      v-if="showprotokol"
      class="navbar-brand"
      to="/"
    >Home
    </router-link>
    <router-link
      v-else
      class="navbar-brand"
      to="/welcome"
      @click.prevent="openSiderbar"
    >welcome
    </router-link>
    <router-link class="navbar-brand" v-if="showprotokol" to="/allrights"
    >Product
    </router-link>
    <router-link class="navbar-brand" v-if="showprotokol" to="/vision"
    >Vision
    </router-link>
    <router-link class="navbar-brand" v-if="showprotokol" to="/aboutus"
    >About us
    </router-link>
    <router-link class="navbar-brand" v-if="showprotokol" to="/contactus"
    >Contact us
    </router-link>
    <router-link class="navbar-brand" v-if="showprotokol" to="https://mynginx.eforsch.com"
    >Vidoes
    </router-link>
    <ul v-if="!currentUser.isLogin" class="list-inline mb-0">
      <li class="list-inline-item">
        <router-link to="/login" class="btn btn-outline-light my-2"
        >LogIn</router-link>
      </li>
      <li class="list-inline-item">
        <router-link to="/register" class="btn btn-outline-light my-2"
        >Register</router-link>
      </li>
    </ul>
    <ul v-else class="list-inline mb-0">
      <li class="list-inline-item">
        <Dropdown>
	  <DropdownItem :class="{'is-disabled': newdisable}">
	    <router-link to="/profile" class="dropdown-item"
	    >Profile
	    </router-link>
	  </DropdownItem>
	  <DropdownItem :class="{'is-disabled': newdisable}">
            <router-link to="/newarticle" class="dropdown-item"
            >New protocol
            </router-link>
	  </DropdownItem>
	  <DropdownItem :class="{'is-disabled': newdisable}">
            <router-link to="/column" class="dropdown-item"
            >Review all protocols
            </router-link>
	  </DropdownItem>
	  <DropdownItem :class="{'is-disabled': newdisable}">
	    <router-link to="/inventory" class="dropdown-item"
	    >Inventory
	    </router-link>
	  </DropdownItem>
		  <!-- <DropdownItem :class="{'is-disabled': editdisable}">
		       <router-link to="/editarticle" class="dropdown-item"
		       >edit article
		       </router-link>
		       </DropdownItem > -->
          <DropdownItem :class="{'is-disabled': logoutdisable}">
            <router-link to="/logout" class="dropdown-item"
            >logout
            </router-link>
	  </DropdownItem>
        </Dropdown>
      </li>
    </ul>
  </nav>
</template>

<script lang="ts">
 import {defineComponent, computed} from "vue";
 import Dropdown from "./Dropdown.vue";
 import DropdownItem from "./DropdownItem.vue";
 import {useStore} from "vuex";

 export default defineComponent({
   name: "GlobalHeader",
   components: {
     Dropdown,
     DropdownItem,
   },
   setup() {
     const store = useStore();
     const currentUser = computed(() => store.state.user);
     const showprotokol = computed(() => store.state.user.isLogin);
     const disableForm = store.state.Header.disable;
     const newdisable = disableForm.newdisable;
     const editdisable = disableForm.editdisable;
     const logoutdisable = disableForm.logoutdisable;
     const openSiderbar = () => {
       store.commit("changeSidebar");
     };
     return {
       currentUser,
       newdisable,
       editdisable,
       logoutdisable,
       openSiderbar,
       showprotokol,
     };
   },
 });
</script>
