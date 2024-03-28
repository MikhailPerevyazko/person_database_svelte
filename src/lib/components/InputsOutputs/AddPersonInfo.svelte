<script setup lang="ts">
  import { AccordionItem, Accordion } from 'flowbite-svelte';
  import { Label, Input } from 'flowbite-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { persons,type Person, type Persons } from '../../../scripts/personsStorage';

  let new_name: String = '';
  let new_surname: String = '';
  let new_middle_name: String = '';
  let string_date: String = '';
  let new_gender_string: String = '';

  function addNewPerson() {
    invoke<Persons>('add_new_info', {data: $persons,
      newName: new_name,
      newSurname: new_surname, 
      newMiddleName: new_middle_name, 
      stringDate: string_date, 
      newGenderString: new_gender_string}).then((updateData) => console.log(persons.set(updateData)))
}
</script>

<div>
  <Accordion class=" w-108">
    <AccordionItem> 
      <span slot="header">Add new info</span>
      <div class="mb-6">
        <Label for="large-input" class="block mb-2">Name</Label>
        <Input id="large-input" class="w-64" size="lg" placeholder="Input new person's name" bind:value={new_name} />
      </div>
      <div class="mb-6">
        <Label for="large-input" class="block mb-2">Surname</Label>
        <Input id="large-input" class="w-64" size="lg" placeholder="Input new person's surname" bind:value={new_surname}/>
      </div>
      <div class="mb-6">
        <Label for="large-input" class="block mb-2">Middle name</Label>
        <Input id="large-input" class="w-64" size="lg" placeholder="Input new person's middle name" bind:value={new_middle_name}/>
      </div>
      <div class="mb-6">
        <Label for="large-input" class="block mb-2">Date of birth DD-MM-YYYY</Label>
        <Input id="large-input" class="w-64" size="lg" placeholder="Input new person's date of birth" bind:value={string_date}/>
      </div>
      <div class="mb-6">
        <form>
          <Label for="large-input" class="block">Gender</Label>
          <select class="w-64" id="underline_select" bind:value={new_gender_string}>
            <option>Male</option>
            <option>Female</option>
          </select>
        </form>
        <div>
          <button class=" mt-4 text-white bg-green-700 justify-center" on:click={addNewPerson}>Add new info</button>
        </div>
      </div>
    </AccordionItem>
  </Accordion>
</div>
  
<style></style>