import { writable } from "svelte/store";

export interface Person {
  id: number;
  name: string;
  surname: string;
  middleName: string;
  dateOfBirth: string;
  gender: boolean;
}
export interface Persons {
  persons: Person[];
}
//* Глобальная переменная с информацией о персонах

export const persons = writable<Persons>();
