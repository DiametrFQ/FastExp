import { Directive, Input } from "@angular/core";

@Directive()
export class SaverComponent {
  @Input()
  public path!: string;
}
