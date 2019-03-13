
#ifndef _FDS_H_
#define _FDS_H_

// Opaque handle to students.
typedef struct student_ {
  char *name;
  int age;
  int year; 
} PhdStudent;

// Opaque handle to professors.
typedef struct professor_ {
  char *name;
  char *favorite_food;
  int likes_nutella; 
} Professor;

// Initializers for students.
PhdStudent NewCody();
PhdStudent NewEgan();
PhdStudent NewKaisheng();
PhdStudent NewKexin();
PhdStudent NewKraft();
PhdStudent NewPT();
PhdStudent NewSahaana();
PhdStudent NewShoumik();
PhdStudent NewDDKang();
PhdStudent NewDeepak();
PhdStudent NewDeepti();
PhdStudent NewFir();
PhdStudent NewJimbo();
PhdStudent NewKeshu();
PhdStudent NewAnkit();
PhdStudent NewSaachi();
PhdStudent NewJustin();
PhdStudent NewSwetha();
PhdStudent NewAnimesh();

// Initializers for professors.
Professor NewMatei();
Professor NewPeter();

#endif
