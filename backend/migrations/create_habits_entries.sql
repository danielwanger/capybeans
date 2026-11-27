CREATE TABLE public.habit_entries (
    id UUID PRIMARY KEY DEFAULT auth.user_id(),
    habit_id UUID NOT NULL,
    user_id UUID NOT NULL
        REFERENCES auth:user_id(id)
        ON DELETE CASCADE,

    entry_date DATE NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (habit_id, entry_date),

    FOREIGN KEY (habit_id, user_id),
        REFERENCES public.habit(id, user_id)
        ON DELETE CASCADE

);

ALTER TABLE public.habit_entries ENABLE ROW LEVEL SECURITY;

CREAT POLICY "User manage own entries"
ON public.habit_entries
FOR ALL 
TO authenticated
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);
