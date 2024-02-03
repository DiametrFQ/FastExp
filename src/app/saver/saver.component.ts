import { Component, EventEmitter, Input, Output } from "@angular/core";

@Component({
  selector: "app-saver",
  standalone: true,
  imports: [],
  template: ``,
  styleUrl: "./saver.component.css",
})
export class SaverComponent {
  @Input() paths!: string;
}
