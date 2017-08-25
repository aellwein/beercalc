import {Injectable} from '@angular/core';
import {Http} from '@angular/http';
import 'rxjs/add/operator/toPromise';
import {Title} from '@angular/platform-browser';

export class Language {
  suffix: string;
  description: string;
}

@Injectable()
export class TranslationService {
  translate: object = {};
  selectedLanguage = 'de';
  languages: Language[];

  constructor(private http: Http,
              private title: Title) {
    this.languages = [
      {
        suffix: 'de',
        description: 'Deutsch'
      },
      {
        suffix: 'en',
        description: 'English'
      },
      {
        suffix: 'ru',
        description: 'Русский'
      }
    ];
    // best guess on browser language
    const language = navigator.language;
    if (language != null) {
      if (language.toLowerCase().indexOf('en') !== -1) {
        this.setLanguage('en');
      } else {
        if (language.toLowerCase().indexOf('ru') !== -1) {
          this.setLanguage('ru');
        } else {
          this.setLanguage('de');
        }
      }
    } else {
      this.setLanguage('de');
    }
  }

  setLanguage(lang: string): void {
    this.selectedLanguage = lang;
    this.http.get('../assets/i18n/lang.' + lang + '.json')
      .toPromise()
      .then(r => {
        this.translate = r.json();
        this.title.setTitle(this.translate['title']);
      });
  }

  getLanguages(): Language[] {
    return this.languages;
  }

  getLanguage(): string {
    return this.selectedLanguage;
  }

  _(key: string): string {
    if (key in this.translate) {
      return this.translate[key];
    } else {
      return 'i18n(' + key + ')';
    }
  }
}
