import {AfterViewChecked, AfterViewInit, Component} from '@angular/core';
import {CommonModule} from '@angular/common';
import {invoke} from "@tauri-apps/api/core";
import {FormsModule} from "@angular/forms";

@Component({
    selector: 'app-root',
    standalone: true,
    imports: [CommonModule, FormsModule],
    templateUrl: './app.component.html',
    styleUrl: './app.component.scss'
})
export class AppComponent implements AfterViewInit{
    computedResult?: number;
    convertedUnit?: string;

    value?: number;
    valueUnit?: string;
    isConverting = false;

    async ngAfterViewInit() {
        this.value = 100;
        this.valueUnit = 'c';

        await this.convert();
    }

    async convert() {
        if (this.isConverting) {
            return;
        }

        this.isConverting = true;

        if (this.valueUnit === 'c') {
            this.computedResult = await invoke<number>("convert_to_f", {value: this.value});
            this.convertedUnit = 'F';
        } else if (this.valueUnit === 'f') {
            this.computedResult = await invoke<number>("convert_to_c", {value: this.value});
            this.convertedUnit = 'C';
        } else {
            throw new Error('Unexpected value unit');
        }

        this.isConverting = false;
    }

    valueUnitChanged(event: any) {
        this.valueUnit = event.target.value;

        if (this.valueUnit === 'c') {
            this.convertedUnit = 'F';
        } else if (this.valueUnit === 'f') {
            this.convertedUnit = 'C'
        } else {
            throw new Error('Unsupported value unit');
        }
    }
}
