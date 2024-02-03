import { Component, EventEmitter, Input, Output } from "@angular/core";
import { SaverComponent } from "../saver.component";

@Component({
  selector: "app-file",
  standalone: true,
  templateUrl: "./file.component.html",
  // styleUrl: "./file.component.css",
})
export class FileComponent extends SaverComponent {
  @Input() path!: string;
  @Output() evem = new EventEmitter<string>();
  moveTo() {
    this.evem.emit(this.path);
  }
}
