<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import {persons,type Person, type Persons} from '../../../scripts/personsStorage'
</script>

<button class="ml-4 text-white bg-green-700">Find by ID</button>