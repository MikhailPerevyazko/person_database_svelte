<script setup lang="ts">
    import { GradientButton } from 'flowbite-svelte';
    import { persons } from "../../../scripts/personsStorage";
</script>

<div>   
    {#if $persons}
    {#each $persons.persons as persons}
    <div class="border-2 border-neutral-600 m-4">
        <div>
            <p> ID: {persons.id}</p>
            <p>Имя: {persons.name}</p> 
            <p>Фамилия: {persons.surname}</p>
            <p>Отчество: {persons.middle_name}</p>
            <p>Дата рождения: {persons.date_of_birth}</p>
            <p>Пол: {persons.gender}}</p>
            <div>
                <GradientButton class="m-4" shadow color="red">Delete</GradientButton>
                <GradientButton class="m-4" shadow color="blue">Change</GradientButton>
            </div>
        </div>
    </div>
    {/each}
    {/if}
</div>
  
<style>
    p {
        margin: 5px;  
    }
    div {
        height: 300px;
        width: 270px;
    }
</style>