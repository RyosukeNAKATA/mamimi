@echo off
cd %1
if exist .python-version (
		mamimi local
)
@echo on
