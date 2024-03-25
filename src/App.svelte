<script lang="ts">
  import SelectFileButton from './lib/components/Buttons/SelectFileButton.svelte'
  import DeleteButton from './lib/components/Buttons/DeleteButton.svelte';
  import FindByIdButton from './lib/components/Buttons/FindByIdButton.svelte';
  import AddPersonInfo from './lib/components/InputsOutputs/AddPersonInfo.svelte';
  import OutputInfo from './lib/components/InputsOutputs/OutputInfo.svelte';
  import Input from './lib/components/InputsOutputs/Input.svelte'
</script>

<main>
  <div class="flex justify-items-center m-10 border-2 border-emerald-600">
    <div class="header space-y-4 m-10">
      <h1 class="mb-4">FIND PERSON</h1>
      <SelectFileButton/>
      <Input/>
      <FindByIdButton/>
      <DeleteButton/>
      <AddPersonInfo/>
    </div>
    <div class="m-10 overflow-auto two-column-div w-96">
      <OutputInfo></OutputInfo>
    </div>
  </div>
</main>

<style></style>