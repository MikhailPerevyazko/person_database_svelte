<script setup lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import {persons,type Person, type Persons} from '../../../scripts/personsStorage';

    let find_id_input: String = '';

    function find() {
        invoke<Persons>('show_info_by_id', {data: $persons, findId: find_id_input}).then((findData) =>
            console.log(persons.set(findData)))
            .catch((err) => 
                console.log(err));
                //alert(err));
    }
 
</script>

<div>
    <input placeholder="Input ID" bind:value={find_id_input}/>
    <button class="ml-4 text-white bg-green-700" on:click={find}>Search</button>
    <button class="ml-4 text-white bg-red-700">Delete</button>
</div>
      
<style></style>

    