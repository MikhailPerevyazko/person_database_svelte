<script setup lang="ts">
import { AccordionItem, Accordion } from 'flowbite-svelte';
import { Label, Input } from 'flowbite-svelte';
</script>

<div>
    <Accordion >
      <AccordionItem>
        <span slot="header">Add new info</span>
        <div class="mb-6">
          <Label for="large-input" class="block mb-2">Name</Label>
          <Input id="large-input" size="lg" placeholder="Input new person's name" />
        </div>
        <div class="mb-6">
          <Label for="large-input" class="block mb-2">Surname</Label>
          <Input id="large-input" size="lg" placeholder="Input new person's surname" />
        </div>
        <div class="mb-6">
          <Label for="large-input" class="block mb-2">Middle name</Label>
          <Input id="large-input" size="lg" placeholder="Input new person's middle name" />
        </div>
        <div class="mb-6">
          <Label for="large-input" class="block mb-2">Date of birth</Label>
          <Input id="large-input" size="lg" placeholder="Input new person's date of birth" />
        </div>
        <div class="mb-6">
          <form class="">
            <Label for="large-input" class="block">Gender</Label>
            <select id="underline_select">
                <option value="true">Male</option>
                <option value="false">Female</option>
            </select>
          </form>
          <button class="m-4 text-white bg-green-700 justify-center">Add new info</button>
        </div>
      </AccordionItem>
    </Accordion>
</div>
  
<style></style>