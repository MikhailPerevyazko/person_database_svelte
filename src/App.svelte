<script lang="ts">
  import SelectFileButton from './lib/components/InputsOutputs/SelectFileButton.svelte'
  import DeleteButton from './lib/components/Buttons/DeleteButton.svelte';
  import FindByIdButton from './lib/components/Buttons/FindByIdButton.svelte';
  import AddPersonInfo from './lib/components/InputsOutputs/AddPersonInfo.svelte';
  import OutputInfo from './lib/components/InputsOutputs/OutputInfo.svelte';
</script>

<main>
  <div class="grid justify-center m-10 items-center">
    <h1 class="mb-4">FIND PERSON</h1>
    <div class="flex items-stretch justify-center ">
      <SelectFileButton/>
      <input class="ml-4" type="text" placeholder="Find by id">
      <FindByIdButton/>
      <DeleteButton/>
    </div>
    <div class="m-10 self-stretch">
      <AddPersonInfo></AddPersonInfo>
    </div>
    <OutputInfo></OutputInfo>
  </div>

</main>

<style></style>