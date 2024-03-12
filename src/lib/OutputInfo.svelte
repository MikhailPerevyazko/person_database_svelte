<script setup lang="ts">
    import { GradientButton } from 'flowbite-svelte';
    import { persons } from "../scripts/personsStorage";
</script>

<div class="m-4 justify-center border border-emerald-400">   
    {#if $persons}
    {#each $persons.persons as persons}
    <div class="m-4 border-4 border-red-500">
        <p class="m-4"> ID: {persons.id}</p>
        <p class="m-4">Имя: {persons.name}</p> 
        <p class="m-4">Фамилия: {persons.surname}</p>
        <p class="m-4">Отчество: {persons.middle_name}</p>
        <p class="m-4">Дата рождения: {persons.date_of_birth}</p>
        <p class="m-4">Пол: {persons.gender}</p>
        <GradientButton class="m-4" shadow color="red">Delete</GradientButton>
    </div>
    {/each}
    {/if}
    
</div>
  
<style></style>