import {Component} from '@angular/core';
import {CommonModule} from '@angular/common';
import {invoke} from "@tauri-apps/api/core";
import {FormsModule} from "@angular/forms";

@Component({
    selector: 'app-root',
    standalone: true,
    imports: [CommonModule, FormsModule],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css'
})
export class AppComponent {
    computedResult?: number;
    convertedUnit?: string;

    value?: number;
    valueUnit?: string;
    isConverting = false;

    async convert() {
        if (this.isConverting) {
            return;
        }

        if (this.valueUnit === 'c') {
            this.convertedUnit = 'F';
            this.computedResult = await invoke<number>("convert_to_f", {value: this.valueUnit});
        } else if (this.valueUnit === 'f') {
            this.convertedUnit = 'C';
            this.computedResult = await invoke<number>("convert_to_c", {value: this.valueUnit});
        } else {
            throw new Error('Unexpected value unit');
        }
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
