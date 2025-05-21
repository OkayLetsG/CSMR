import { Component } from '@angular/core';
import { FoldersComponent } from './folders/folders.component';
import { EditorComponent } from './editor/editor.component';
import { SnippetsComponent } from './snippets/snippets.component';
import { NzSplitterModule } from 'ng-zorro-antd/splitter';

@Component({
  selector: 'app-welcome',
  templateUrl: './welcome.component.html',
  styleUrl: './welcome.component.scss',
  imports: [FoldersComponent, EditorComponent, SnippetsComponent, NzSplitterModule]
})
export class WelcomeComponent {

}
