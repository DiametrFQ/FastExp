import { Component, EventEmitter, Input, Output } from "@angular/core";
import { SaverComponent } from "../saver.component";

@Component({
  selector: "app-file",
  standalone: true,
  templateUrl: "./file.component.html",
})
export class FileComponent extends SaverComponent {}
