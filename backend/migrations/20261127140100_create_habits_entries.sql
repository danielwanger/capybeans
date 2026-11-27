CREATE TABLE public.habit_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    habit_id UUID NOT NULL,
    user_id UUID NOT NULL
        REFERENCES auth.user(id)
        ON DELETE CASCADE,

    entry_date DATE NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (habit_id, entry_date),

    FOREIGN KEY (habit_id, user_id)
        REFERENCES public.habits(id, user_id)
        ON DELETE CASCADE

);

ALTER TABLE public.habit_entries ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users manage own entries"
ON public.habit_entries
FOR ALL 
TO authenticated
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);
