<template>
  <div class="dropdown" ref="dropdownRef">
    <a href="#" class="btn btn-outline-light my-2 dropdown-toggle" @click.prevent="toggleOpen">
      welcome {{ name }}
    </a>
    <ul class="dropdown-menu" :style="{display: 'block'}" v-if="isOpen">
      <slot></slot>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch, computed } from 'vue'
import useClickOutside from '../../hooks/useClickOutside'
export default defineComponent({
  name: 'Dropdown',
  setup () {
    const name = window.sessionStorage.getItem("username")
    const dropdownRef = ref<null | HTMLElement>(null)
    const isOpen = ref(false)
    const toggleOpen = () => {
      isOpen.value = !isOpen.value
    }
    const isClickOutside = useClickOutside(dropdownRef)
    watch(isClickOutside, () => {
      if (isOpen.value && isClickOutside.value) {
 	isOpen.value = false
      }
    })
    return {
      name,
      isOpen,
      toggleOpen,
      dropdownRef
    }
  }
})
</script>
