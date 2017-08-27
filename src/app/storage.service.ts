import {Injectable} from '@angular/core';

@Injectable()
export class StorageService {

  storageReplacement: object;

  constructor() {
  }

  clear(): void {
    if (typeof (localStorage) !== 'undefined') {
      localStorage.clear();
    } else {
      this.storageReplacement = {};
    }
  }

  getItem(key: string): string {
    if (typeof (localStorage) !== 'undefined') {
      return localStorage.getItem(key);
    } else {
      return this.storageReplacement[key];
    }
  }

  hasItem(key: string): boolean {
    if (typeof (localStorage) !== 'undefined') {
      return !(localStorage.getItem(key) === null);
    } else {
      return key in this.storageReplacement;
    }
  }

  setItem(key: string, data: string): void {
    if (typeof (localStorage) !== 'undefined') {
      localStorage.setItem(key, data);
    } else {
      this.storageReplacement[key] = data;
    }
  }

  removeItem(key: string): void {
    if (typeof (localStorage) !== 'undefined') {
      localStorage.removeItem(key);
    } else {
      delete this.storageReplacement[key];
    }
  }
}
