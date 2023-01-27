<script lang="ts">
    import { TextInput } from '@svelteuidev/core';
    export let value: string;
    export let label: string;
    export let invalid: boolean = false
  </script>
  
  <TextInput bind:value
    placeholder="2022-12-19"
    label={label}
    {invalid}
  />
<!-- я че должен все свойства TextInput импортировать?  -->
