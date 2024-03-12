<script setup lang="ts">
    import { GradientButton } from 'flowbite-svelte';
    import { persons } from "../scripts/personsStorage";
</script>

<div class="justify-center">   
    {#if $persons}
    {#each $persons.persons as persons}
    <div class="m-4 border-2 border-neutral-600">
        <p> ID: {persons.id}</p>
        <p>Имя: {persons.name}</p> 
        <p>Фамилия: {persons.surname}</p>
        <p>{persons.middle_name}</p>
        <p>Дата рождения: {persons.date_of_birth}</p>
        <p>Пол: {persons.gender}</p>
        <GradientButton class="m-4" shadow color="red">Delete</GradientButton>
        <GradientButton class="m-4" shadow color="blue">Change</GradientButton>
    </div>
    {/each}
    {/if}
</div>
  
<style>
    p {
        margin: 16px;
    }
</style>