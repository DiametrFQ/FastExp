import { Component, OnInit } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/tauri";
import { DirComponent } from "./saver/dir/dir.component";
import { FileComponent } from "./saver/file/file.component";

@Component({
  selector: "app-root",
  styleUrl: "./app.component.css",
  templateUrl: "./app.component.html",
  standalone: true,
  imports: [FormsModule, DirComponent, FileComponent],
})
export class AppComponent implements OnInit {
  isLoad: boolean = true;
  data: [[string, string]] = [["a", "a"]];
  dataToShow: [string, string][] = [["a", "a"]];
  pathsMainDirectory: [string[], string[]] = [[], []];
  currentPosition: number = 0;
  mainDirectory = "C:/";
  seacrhWord = "";

  ngOnInit() {
    this.getPathsFromMainDirectory();
  }

  async saveData() {
    try {
      console.log("asdad");
      invoke("save_paths_from", { directory: "/" });

      this.isLoad = false;
    } catch (error) {
      console.error("Error fetching data", error);
    }
  }

  async loadData() {
    try {
      this.data = await invoke<[[string, string]]>("text_file", {
        path: "./files.txt",
      });
      this.dataToShow = this.data;
      this.isLoad = false;
      // setInterval(() => this.dataToShow.push(this.data.shift()!), 0.0001);
    } catch (error) {
      console.error("Error fetching data", error);
    }
  }

  // showNextPath() {
  //   if (this.currentPosition < this.data.length) {
  //     this.dataToShow.push(this.data[this.currentPosition]);
  //     this.currentPosition++;
  //     // new Promise(() => this.showNextPath());
  //     // Повторно вызывайте функцию через 1000 миллисекунд (1 секунда)
  //     setTimeout(() => this.showNextPath(), 0.0001);
  //     // new Promise(this.showNextPath);
  //   }
  //   this.data.map((item) => {
  //     this.dataToShow.push(item);
  //   });
  //   this.dataToShow = this.data;
  // }

  async getPaths() {
    // Получите данные перед их отображением
    await this.saveData();

    // // Начните показывать пути
    // this.showNextPath();
  }

  async getPathsFromMainDirectory() {
    this.pathsMainDirectory = await invoke<[string[], string[]]>(
      "get_files_from_dir",
      {
        targetDirectory: this.mainDirectory,
      }
    );
  }

  trackByFn(index: number, item: string[]): string {
    // Возвращайте уникальный идентификатор для каждого элемента
    return `${item[0]}_${item[1]}`;
  }

  choisePaths(paths: string) {
    this.mainDirectory += paths + "/";
    this.getPathsFromMainDirectory();
  }

  searchSaver() {
    const regexp = new RegExp(this.seacrhWord.toLowerCase());
    const arrSearch = this.data
      .map((arr) => ((arr[1] = arr[1].toLowerCase()), arr))
      .filter((arr: [string, string]) => arr[1].search(regexp) + 1);
    this.dataToShow = arrSearch;
  }
}
