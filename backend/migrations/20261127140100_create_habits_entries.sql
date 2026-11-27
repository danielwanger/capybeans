CREATE TABLE public.habits_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    habit_id UUID NOT NULL,
    user_id UUID NOT NULL
        REFERENCES auth.users(id)
        ON DELETE CASCADE,

    entry_date DATE NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (habit_id, entry_date),

    FOREIGN KEY (habit_id, user_id)
        REFERENCES public.habits(id, user_id)
        ON DELETE CASCADE

);

ALTER TABLE public.habits_entries ENABLE ROW LEVEL SECURITY; /* Zugriff auf die Tabelle nur für authentifizierte Benutzer */

/* jeder Benutzer kann nur seine eigenen Einträge verwalten */
CREATE POLICY "Users manage own entries"
ON public.habits_entries
FOR ALL 
TO authenticated
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);
