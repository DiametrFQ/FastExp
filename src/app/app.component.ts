import { NgFor, NgIf } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  styleUrl: "./app.component.css",
  imports: [NgIf, NgFor],
  templateUrl: "./app.component.html",
  standalone: true,
})
export class AppComponent implements OnInit {
  isLoad: boolean = true;
  data: [[string, string]] = [["a", "a"]];
  dataToShow: [[string, string]] = [["a", "a"]];
  currentPosition: number = 0;

  ngOnInit() {
    // this.getPaths();
  }

  async saveData() {
    try {
      invoke("save_paths_from", { path: "/" });

      this.isLoad = false;
    } catch (error) {
      console.error("Error fetching data", error);
    }
  }

  async loadData() {
    try {
      this.dataToShow = await invoke<[[string, string]]>("text_file", {
        path: "./files.txt",
      });
      this.isLoad = false;
      // setInterval(() => this.dataToShow.push(this.data.shift()!), 0.0001);
    } catch (error) {
      console.error("Error fetching data", error);
    }
  }

  showNextPath() {
    if (this.currentPosition < this.data.length) {
      this.dataToShow.push(this.data[this.currentPosition]);
      this.currentPosition++;
      // new Promise(() => this.showNextPath());
      // Повторно вызывайте функцию через 1000 миллисекунд (1 секунда)
      setTimeout(() => this.showNextPath(), 0.0001);
      // new Promise(this.showNextPath);
    }
    this.data.map((item) => {
      this.dataToShow.push(item);
    });
    this.dataToShow = this.data;
  }

  async getPaths() {
    // Получите данные перед их отображением
    await this.saveData();

    // Начните показывать пути
    this.showNextPath();
  }

  trackByFn(index: number, item: string[]): string {
    // Возвращайте уникальный идентификатор для каждого элемента
    return `${item[0]}_${item[1]}`;
  }
}
