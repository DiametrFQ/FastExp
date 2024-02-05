import { Component, EventEmitter, Input, Output } from "@angular/core";
import { SaverComponent } from "../saver.component";
@Component({
  selector: "app-dir",
  standalone: true,
  templateUrl: "./dir.component.html",
  styleUrl: "./dir.component.css",
})
export class DirComponent extends SaverComponent {
  @Output()
  evem = new EventEmitter<string>();

  moveTo() {
    this.evem.emit(this.path);
  }
}
