<script setup lang="ts">
  import { save } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import {persons,type Person, type Persons} from '../scripts/personsStorage'
  
  async function openSaveDialog() {
    const filePath = await save({
      filters: [
        {
          name: "Yaml",
          extensions: ["yml", "yaml"],
        },
      ],
    });
    
    invoke<Persons>("open_db", { filePath: filePath }).then((data)=>{ 

      persons.set(data);  
      console.log($persons);

    });
  }
</script>

<div>
  <button class="ml-4" on:click={openSaveDialog}>Select a file</button>
</div>

<style></style>
  