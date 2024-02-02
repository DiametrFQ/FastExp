import { Component, EventEmitter, Input, Output } from "@angular/core";

@Component({
  selector: "app-saver",
  standalone: true,
  imports: [],
  templateUrl: "./saver.component.html",
  styleUrl: "./saver.component.css",
})
export class SaverComponent {
  @Input() paths!: string;
  @Output() evem = new EventEmitter<string>();
  moveTo() {
    this.evem.emit(this.paths);
  }
}
