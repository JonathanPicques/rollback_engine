import * as monaco from 'monaco-editor';
import {useMeasure} from 'react-use';
import {useRef, useEffect, useCallback} from 'react';
import type {VFC} from 'react';

monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
    noEmit: true,
    target: monaco.languages.typescript.ScriptTarget.ES2020,
    strict: true,
    strictNullChecks: true,
    allowNonTsExtensions: true,
});
monaco.languages.typescript.typescriptDefaults.setDiagnosticsOptions({
    noSyntaxValidation: false,
    noSemanticValidation: false,
});

interface CodeEditorProps {
    name: string;
    value: string;
    options?: monaco.editor.IEditorOptions;
    autoFocus?: boolean;
    className?: string;
    //
    onBlur?: () => void;
    onChange?: (value: string) => void;
    onEditorInit?: (editor: monaco.editor.IStandaloneCodeEditor) => void;
}

export const Script: VFC<CodeEditorProps> = ({name, value, options, autoFocus, className, onBlur, onChange, onEditorInit}) => {
    const editorRef = useRef<monaco.editor.IStandaloneCodeEditor>();
    const layoutRef = useRef<HTMLDivElement>();
    const preventChangeRef = useRef(false);
    const [measureRef, measureRect] = useMeasure();

    const combineRef = useCallback(
        (element: HTMLDivElement) => {
            measureRef(element);
            if (!layoutRef.current) {
                const uri = monaco.Uri.parse(`file:///${name.replace('script::', '')}.ts`);
                layoutRef.current = element;
                editorRef.current = monaco.editor.create(layoutRef.current, {
                    model: monaco.editor.getModel(uri) || monaco.editor.createModel(value, 'typescript', uri),
                    theme: 'vs-dark',
                    language: 'typescript',
                });
                //
                if (autoFocus) {
                    editorRef.current.focus();
                }
                onEditorInit?.(editorRef.current);
            }
        },
        [name, value, autoFocus, measureRef, onEditorInit],
    );

    useEffect(() => {
        return () => {
            preventChangeRef.current = true;
            editorRef.current && editorRef.current.dispose();
        };
    }, []);

    useEffect(() => {
        if (editorRef.current) {
            const model = editorRef.current.getModel()!;
            if (model.getValue() !== value) {
                preventChangeRef.current = true;
                model.pushEditOperations(
                    [],
                    [
                        {
                            range: model.getFullModelRange(),
                            text: value,
                        },
                    ],
                    () => null,
                );
                preventChangeRef.current = false;
            }
        }
    }, [value]);

    useEffect(() => {
        if (options && editorRef.current) {
            const disposable = editorRef.current.onDidChangeConfiguration(() => {
                setImmediate(() => monaco.editor.remeasureFonts());
            });
            editorRef.current.updateOptions(options);
            return () => disposable.dispose();
        }
        return undefined;
    }, [options]);

    useEffect(() => {
        if (editorRef.current && measureRect) {
            editorRef.current.layout();
        }
    }, [measureRect]);

    useEffect(() => {
        if (editorRef.current) {
            const disposable = editorRef.current.onDidBlurEditorWidget(() => {
                onBlur && onBlur();
            });
            return () => disposable.dispose();
        }
    }, [onBlur]);

    useEffect(() => {
        if (editorRef.current) {
            const disposable = editorRef.current.onDidChangeModelContent(() => {
                if (!preventChangeRef.current) {
                    onChange?.(editorRef.current!.getValue());
                }
            });
            return () => disposable.dispose();
        }
    }, [onChange]);

    return <div ref={combineRef} style={{flex: 1, width: '100%', height: '100%'}} className={className} />;
};
