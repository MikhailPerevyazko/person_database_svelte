<script lang="ts">
  import SelectFileButton from './lib/components/Buttons/SelectFileButton.svelte'
  import DeleteButton from './lib/components/Buttons/DeleteButton.svelte';
  import FindByIdButton from './lib/components/Buttons/FindByIdButton.svelte';
  import AddPersonInfo from './lib/components/InputsOutputs/AddPersonInfo.svelte';
  import OutputInfo from './lib/components/InputsOutputs/OutputInfo.svelte';
</script>

<main>
  <div class="m-10 grid grid-rows-10 gap-4">
    <div class="header space-y-4 border-2 border-emerald-600 rows-span-2 m-10">
      <h1 class="mb-4">FIND PERSON</h1>
      <SelectFileButton/>
      <input class="ml-4 rounded-lg" type="text" placeholder="Enter the ID">
      <FindByIdButton/>
      <DeleteButton/>
      <AddPersonInfo/>
    </div>
    <div class="rows-span-6">
      <OutputInfo></OutputInfo>
    </div>
  </div>
</main>

<style>
   /* .header {
    position: fixed;
  }  */
</style>