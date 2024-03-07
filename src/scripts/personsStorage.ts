import { writable } from "svelte/store";

export interface Person {
  id: number;
  name: string;
  surname: string;
  middle_name: string;
  date_of_birth: string;
  gender: boolean;
}
export interface Persons extends Array<Person> {}
//* Глобальная переменная с информацией о персонах.
export const persons = writable<Persons>();
