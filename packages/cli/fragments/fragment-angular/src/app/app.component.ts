import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  template: `
    <div class="container">
      <h1>Welcome to Tauri + Angular!</h1>

      <div class="row">
        <a href="https://tauri.app" target="_blank">
          <img src="/assets/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://angular.io" target="_blank">
          <img
            src="/assets/angular.svg"
            class="logo angular"
            alt="Angular logo"
          />
        </a>
      </div>

      <p>Click on the logos to learn more about the frameworks</p>

      <div class="row">
        <div>
          <input #greetInput id="greet-input" placeholder="Enter a name..." />
          <button type="button" (click)="greet(greetInput.value)">Greet</button>
        </div>
      </div>

      <p>{{ greetingMessage }}</p>
    </div>
  `,
  styles: [
    `
      .logo.angular:hover {
        filter: drop-shadow(0 0 2em #e32727);
      }
    `,
  ],
  standalone: true,
})
export class AppComponent {
  greetingMessage = "";

  greet(name: string): void {
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }
}
